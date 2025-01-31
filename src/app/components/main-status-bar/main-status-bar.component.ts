import { Component, OnInit } from '@angular/core';
import { SleepService } from '../../services/sleep.service';
import { VRChatService } from '../../services/vrchat.service';
import { UserStatus } from 'vrchat/dist';
import { hshrink, noop } from '../../utils/animations';
import { firstValueFrom } from 'rxjs';
import { OpenVRService } from '../../services/openvr.service';
import { BackgroundService } from '../../services/background.service';

@Component({
  selector: 'app-main-status-bar',
  templateUrl: './main-status-bar.component.html',
  styleUrls: ['./main-status-bar.component.scss'],
  animations: [hshrink(), noop()],
})
export class MainStatusBarComponent implements OnInit {
  sleepMode = this.sleepService.mode;
  user = this.vrchat.user;

  constructor(
    private sleepService: SleepService,
    private vrchat: VRChatService,
    protected openvr: OpenVRService,
    protected background: BackgroundService
  ) {}

  ngOnInit(): void {}

  getStatusColor(status: UserStatus) {
    switch (status) {
      case UserStatus.Active:
        return 'var(--color-vrchat-status-green)';
      case UserStatus.JoinMe:
        return 'var(--color-vrchat-status-blue)';
      case UserStatus.AskMe:
        return 'var(--color-vrchat-status-orange)';
      case UserStatus.Busy:
        return 'var(--color-vrchat-status-red)';
      case UserStatus.Offline:
        return 'black';
    }
  }

  async toggleSleepMode() {
    if (await firstValueFrom(this.sleepService.mode)) {
      await this.sleepService.disableSleepMode({ type: 'MANUAL' });
    } else {
      await this.sleepService.enableSleepMode({ type: 'MANUAL' });
    }
  }
}
